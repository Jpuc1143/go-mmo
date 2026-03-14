import {
  GameServerMessage,
  InvalidMoveMesage,
  StonePlacedMessage,
} from "../Gateway/GameServerMessage";
import GameGateway from "../Gateway/GameGateway";
import GameRenderer from "../View/GameRenderer";
import { Color } from "./Color";
import { Coord, CoordHash } from "./Coord";
import { GroupId, Group } from "./Group";
import { PlaceStoneCommand } from "../Gateway/GameClientCommand";
import { BoardDataMessage } from "../Gateway/GameServerMessage";

export class Game {
  readonly playerColor: Color;
  renderer!: GameRenderer;
  readonly gateway: GameGateway;

  readonly groups: Map<GroupId, Group> = new Map();

  readonly pendingMoves: Set<CoordHash> = new Set();

  public constructor(gateway: GameGateway) {
    this.gateway = gateway;
    if (Math.random() > 0.5) {
      this.playerColor = Color.Black;
    } else {
      this.playerColor = Color.White;
    }
  }

  public async start(renderer: GameRenderer) {
    this.renderer = renderer;
    this.gateway.start(this);
  }

  public receiveServerMessage(message: GameServerMessage) {
    switch (message.type) {
      case "StonePlaced":
        this.receiveStonePlaced(message);
        break;

      case "BoardData":
        this.receiveBoardData(message);
        break;

      case "InvalidMove":
        this.receiveRollback(message);
        break;

      default:
        console.log("Unknown server message type");
    }
  }

  receiveStonePlaced(message: StonePlacedMessage) {
    if (!this.groups.has(message.assigned_group.id)) {
      const { id, color } = message.assigned_group;
      const group = new Group(id, color);
      this.groups.set(id, group);
    }

    // TODO workaround for using coercion in JSON
    const coord = new Coord(message.coord.x, message.coord.y);

    const group = this.groups.get(message.assigned_group.id)!;
    group.stones.push(coord);
    this.pendingMoves.delete(coord.hash());
    this.renderer.placeStone(coord, group.color);

    for (const id of message.captured_groups_ids) {
      this.captureGroup(id);
    }

    // TODO merge groups
    for (const id of message.merged_groups_ids) {
      this.mergeGroup(group.id, id);
    }
  }

  captureGroup(id: GroupId) {
    const group = this.groups.get(id);
    if (group === undefined) return;

    for (const coord of group.stones) {
      this.renderer.removeStone(coord);
    }

    this.groups.delete(id);
  }

  mergeGroup(targetId: GroupId, id: GroupId) {
    const targetGroup = this.groups.get(targetId);
    if (targetGroup === undefined) return;

    const group = this.groups.get(id);
    if (group === undefined) return;

    targetGroup.stones.push(...group.stones);
    this.groups.delete(id);
  }

  receiveBoardData(message: BoardDataMessage) {
    for (const { id, color, stones } of message.grouped_stones) {
      const group = this.groups.get(id) || new Group(id, color);
      for (let stone of stones) {
        stone = new Coord(stone.x, stone.y);
        group.stones.push(stone);
        this.renderer.placeStone(stone, color);
      }
      this.groups.set(id, group);
    }
  }

  receiveRollback(message: InvalidMoveMesage) {
    const coord = new Coord(message.coord.x, message.coord.y);
    if (this.pendingMoves.has(coord.hash())) {
      this.pendingMoves.delete(coord.hash());
      this.renderer.removeStone(coord);
      console.log("Rolling back", coord);
    }
  }

  public placeStone(coord: Coord) {
    const cmd = new PlaceStoneCommand(coord, this.playerColor);
    this.pendingMoves.add(coord.hash());
    this.renderer.placeStone(coord, this.playerColor);
    this.gateway.sendCommand(cmd);
  }
}

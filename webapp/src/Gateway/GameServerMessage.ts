import { Coord } from "../Domain/Coord";
import { GroupId } from "../Domain/Group";
import { GroupDto } from "./GroupDto";
import { GroupedStonesDto } from "./GroupedStonesDto";

export type GameServerMessage = BoardDataMessage | StonePlacedMessage;

export interface BoardDataMessage {
  type: "BoardData";
  grouped_stones: Array<GroupedStonesDto>;
}

export interface StonePlacedMessage {
  type: "StonePlaced";
  coord: Coord;
  assigned_group: GroupDto;
  captured_groups_ids: Array<GroupId>;
  merged_groups_ids: Array<GroupId>;
}

import { Color } from "../Domain/Color";
import { Coord } from "../Domain/Coord";

export type GameClientCommand = PlaceStoneCommand;

export class PlaceStoneCommand {
  readonly type = "PlaceStone";
  readonly coord: Coord;
  readonly color: Color;

  constructor(coord: Coord, color: Color) {
    this.coord = coord;
    this.color = color;
  }
}

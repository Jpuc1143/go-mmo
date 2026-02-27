import { Color } from "./Color";
import { Coord } from "./Coord";

export type GroupId = number;

export class Group {
  readonly id: GroupId;
  readonly color: Color;
  stones: Array<Coord> = [];

  constructor(id: GroupId, color: Color) {
    this.id = id;
    this.color = color;
  }
}

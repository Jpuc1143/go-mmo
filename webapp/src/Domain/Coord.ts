export type CoordHash = string;

export class Coord {
  readonly x: number;
  readonly y: number;

  public constructor(x: number, y: number) {
    this.x = x;
    this.y = y;
  }

  public hash(): CoordHash {
    return this.x + ":" + this.y;
  }
}

import { Color } from "../Domain/Color";
import { Coord } from "../Domain/Coord";

export interface GroupedStonesDto {
  id: number;
  color: Color;
  stones: Array<Coord>;
}

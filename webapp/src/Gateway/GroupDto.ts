import { Color } from "../Domain/Color";
import { GroupId } from "../Domain/Group";

export interface GroupDto {
  id: GroupId;
  color: Color;
}

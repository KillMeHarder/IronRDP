import {Guid} from "guid-typescript";
import {DesktopSize} from "./desktop-size";

export class Session {
  id:Guid;
  sessionId: number;
  name?:string;
  active: boolean;
  desktopSize: DesktopSize;

  constructor(name?: string) {
    this.id = Guid.create();
    this.name = name;
  }
}

import {Injectable} from "@angular/core";
import {Observable, Subject} from "rxjs";

export interface ServerRect {
  free(): void;

  clone_buffer(): Uint8Array;

  bottom: number;
  left: number;
  right: number;
  top: number;
}

export interface NewSessionInfo {
  session_id: number,
  websocket_port: number,
  initial_desktop_size: DesktopSize,
}

export interface DesktopSize {
  width: number,
  height: number
}

export interface ResizeEvent {
  session_id: number,
  desktop_size: DesktopSize,
}

@Injectable()
export abstract class ServerBridgeService {
  abstract init(): void;

  abstract connect(username: string, password: string, address: string): Observable<NewSessionInfo>;

  abstract resize: Observable<ResizeEvent>;

  abstract updateImage: Observable<any>;

  abstract updateMouse(mouse_x: number, mouse_y: number, click_state: number): void;
}


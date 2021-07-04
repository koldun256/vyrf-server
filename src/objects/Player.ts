import { Socket } from "socket.io";

export class Player {
  constructor(
    private socket: Socket,
    private room: Room
  ){}
}

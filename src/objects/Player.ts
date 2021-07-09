import { Socket } from "socket.io";
import { Room } from '../room/Room'
 
export class Player {
  constructor(
    private socket: Socket,
    private room: Room
  ){}
}

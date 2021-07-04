import { Socket } from "socket.io"
import { Player } from "../objects/Player"
import { RoomManager } from "../room/RoomManager"

export class User {
  constructor(
    private socket: Socket,
    roomManager: RoomManager
  ){
    socket.on('room_enter', () => {
      const room: Room = roomManager.getWaiting()
      new Player(socket, room)
    })
  }
}

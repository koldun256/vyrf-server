import { Socket } from "socket.io"

export class User {
  constructor(
    private socket: Socket
  ){}
}

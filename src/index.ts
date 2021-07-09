import { Server } from 'socket.io'
import { RoomManager } from './room/RoomManager'
import { User } from './user/User'

const io = new Server()

const roomManager = new RoomManager()

io.on('connection', socket => new User(socket, roomManager))

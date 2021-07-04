import { Server } from 'socket.io'
import { User } from './user/User'

const io = new Server()
io.on('connection', socket => new User(socket))

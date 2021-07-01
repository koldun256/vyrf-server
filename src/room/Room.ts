import { RoomManager } from './RoomManager'
import { getId } from '../utils'

export class Room {
	public id: string = getId()

	constructor(
		private roomManager: RoomManager
	) {}
}

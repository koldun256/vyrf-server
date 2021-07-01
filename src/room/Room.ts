import { RoomManager } from './RoomManager'
import { getId } from '../utils'

export class Room {
	public id: string = getId()
	public full: boolean = false

	constructor(
		private roomManager: RoomManager
	) {}
}

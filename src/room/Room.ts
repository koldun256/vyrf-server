import { RoomManager } from './RoomManager'

export class Room {
	public id: string

	constructor(
		private roomManager: RoomManager
	) {}
}

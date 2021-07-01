import { Room } from './Room'

export class RoomManager {
	private rooms: Record<string, Room> = {}

	public addRoom(): Room {
		let room = new Room(this)
		this.rooms[room.id] = room
		return room
	}

	public getById(id: string): Room | undefined {
		return this.rooms[id]
	}
}

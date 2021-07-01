import { Room } from './Room'

export class RoomManager {
	private rooms: Record<string, Room> = {}
	private waiting: Room = null

	public addRoom(): Room {
		let room = new Room(this)
		this.rooms[room.id] = room
		return room
	}

	public getById(id: string): Room | undefined {
		return this.rooms[id]
	}

	public getWaiting(): Room {
		return this.waiting.full
			? this.waiting = this.addRoom()
			: this.waiting 
	}
}

export const getId = (length = 16) => Array
		.from({ length })
		.map(() => 
			Math.floor(Math.random() * 32).toString(32)
		)
		.join('')

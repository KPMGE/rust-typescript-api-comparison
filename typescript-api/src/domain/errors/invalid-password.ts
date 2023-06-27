export class InvalidPasswordError extends Error {
  constructor() {
    super('passwords must have at least 5 characters!')
    this.name = 'InvalidPasswordError'
  }
}

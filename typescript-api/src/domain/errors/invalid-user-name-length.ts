export class InvalidUserNameLenghtError extends Error {
  constructor(min: number) {
    super(`user names must have at least ${min} characters!`)
    this.name = 'InvalidUserNameLenghtError'
  }
}

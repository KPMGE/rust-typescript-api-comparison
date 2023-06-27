export class DuplicateUserNameError extends Error {
  constructor(name: string) {
    super(`name '${name}' already taken!`)
    this.name = 'DuplicateUserNameError'
  }
}

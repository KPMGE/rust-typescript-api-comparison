import { DeleteUserRepository } from "../repositories"

export class DeleteUserService {
  constructor(
    private readonly repo: DeleteUserRepository
  ) { }

  async delete(userId: number): Promise<void> {
    await this.repo.delete(userId)
  }
}

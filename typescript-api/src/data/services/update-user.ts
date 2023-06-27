import { User } from "../../domain/entities";
import { UpdateUserRepository } from "../repositories";

export class UpdateUserService {
  constructor(
    private readonly repo: UpdateUserRepository
  ) {}

  async update(userId: number, newUser: Omit<User, 'id' | 'password'>): Promise<void> {
    this.repo.update(userId, newUser)
  }
}

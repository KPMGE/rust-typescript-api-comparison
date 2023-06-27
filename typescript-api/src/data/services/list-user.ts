import { User } from "../../domain/entities";
import { ListUserRepository } from "../repositories";

export class ListUserService {
  constructor(
    private readonly repo: ListUserRepository
  ) {}

  async list(): Promise<User[]> {
    const users = await this.repo.list()
    return users
  }
}

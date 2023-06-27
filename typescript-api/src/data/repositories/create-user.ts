import { User } from "../../domain/entities";

export interface CreateUserRepository {
  create(user: User): Promise<void>
}

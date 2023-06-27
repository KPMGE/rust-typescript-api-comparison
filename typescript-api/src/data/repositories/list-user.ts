import { User } from "../../domain/entities";

export interface ListUserRepository {
  list(): Promise<User[]>
}

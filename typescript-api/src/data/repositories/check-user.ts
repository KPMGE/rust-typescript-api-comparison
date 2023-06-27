import { User } from "../../domain/entities";

export interface CheckUserRepository {
  exists(user: User): Promise<boolean>
}

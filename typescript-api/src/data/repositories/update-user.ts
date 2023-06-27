import { User } from "../../domain/entities";

export interface UpdateUserRepository {
  update(userId: number, newUser: Omit<User, 'id' | 'password'>): Promise<void>
}

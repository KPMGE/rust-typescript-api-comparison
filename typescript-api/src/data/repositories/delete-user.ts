export interface DeleteUserRepository {
  delete(userId: number): Promise<void>
}

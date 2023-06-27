import { DeleteTodoRepository } from "../repositories";

export class DeleteTodoService {
  constructor(
    private readonly repo: DeleteTodoRepository
  ) {}

  async delete(todoId: number): Promise<void> {
    this.repo.delete(todoId)
  }
}

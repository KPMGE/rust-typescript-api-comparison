import { Todo } from "../../domain/entities";
import { UpdateTodoRepository } from "../repositories";

export class UpdateTodoService {
  constructor(
    private readonly repo: UpdateTodoRepository
  ) {}

  async update(todo: Todo): Promise<void> {
    await this.repo.update(todo)
  }
}

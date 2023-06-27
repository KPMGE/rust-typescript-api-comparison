import { Todo } from "../../domain/entities";
import { ListTodoRepository } from "../repositories";

export class ListTodoService {
  constructor(
    private readonly repo: ListTodoRepository
  ) {}

  async list(userId: number): Promise<Todo[]> {
    return await this.repo.list(userId)
  }
}

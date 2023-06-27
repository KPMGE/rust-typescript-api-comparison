import { Todo } from "../../domain/entities";
import { CreateTodoRepository } from "../repositories";

export class CreateTodoService {
  constructor(
    private readonly repo: CreateTodoRepository
  ) {}

  async create(userId: number, todo: Omit<Todo, 'id'>): Promise<void> {
    await this.repo.create(userId, todo)
  }
}

export interface DeleteTodoRepository {
  delete(todoId: number): Promise<void>
}

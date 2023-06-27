import { Todo } from "../../domain/entities";
import { CreateTodoService } from "../../data/services";
import { Request, Response } from 'express'
import httpStatus from 'http-status-codes'

export class CreateTodoController {
  constructor(
    private readonly service: CreateTodoService
  ) {}

  async handle(req: Request, res: Response) {
    const userId = Number(req.params.userId)
    const todo: Omit<Todo, 'id'> = {
      title: req.body.title,
      description: req.body.description,
      completed: req.body.completed || false
    }

    try {
      await this.service.create(userId, todo)
      return res.status(httpStatus.CREATED).send()
    } catch (error) {
      if (process.env.SHOW_ERRORS) {
        console.error('ERROR: ', error)
      }

      return res.status(httpStatus.INTERNAL_SERVER_ERROR).json({
        message: 'internal server error'
      })
    }
  }
}

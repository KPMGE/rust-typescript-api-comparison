import { Request, Response } from 'express'
import { UpdateTodoService } from "../../data/services";
import httpStatus from 'http-status-codes'
import { Todo } from '../../domain/entities';

export class UpdateTodoController {
  constructor(
    private readonly service: UpdateTodoService
  ) {}

  async handle(req: Request, res: Response) {
    const todo: Todo = {
      id: Number(req.params.tagId),
      completed: req.body.completed || false,
      description: req.body.description,
      title: req.body.title
    }

    try {
      await this.service.update(todo)
      return res.status(httpStatus.OK).send()
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

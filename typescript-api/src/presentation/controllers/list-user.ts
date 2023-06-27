import { Request, Response } from 'express'
import { ListUserService } from "../../data/services";
import httpStatus from 'http-status-codes'

export class ListUserController {
  constructor(
    private readonly service: ListUserService
  ) { }

  async handle(_req: Request, res: Response) {
    try {
      const users = await this.service.list()
      return res.status(httpStatus.OK).json(users)
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

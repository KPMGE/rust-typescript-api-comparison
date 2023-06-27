import { Request, Response } from 'express'
import { DeleteUserService } from "../../data/services";
import httpStatus from 'http-status-codes'

export class DeleteUserController {
  constructor(
    private readonly service: DeleteUserService
  ) { }

  async handle(req: Request, res: Response) {
    const userId = Number(req.params.userId)

    try {
      await this.service.delete(userId)
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

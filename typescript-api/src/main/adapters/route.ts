import { Request, Response } from 'express'

export const adaptRoute = (controller: any) => {
  return async (req: Request, res: Response) => {
    controller.handle(req, res)
  }
}

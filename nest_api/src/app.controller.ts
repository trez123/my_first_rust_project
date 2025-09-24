import {
  Controller,
  UploadedFile,
  Post,
  UseInterceptors,
  Res,
} from '@nestjs/common';
import { Response } from 'express';
import { AppService } from './app.service';
import { ApiBody, ApiConsumes, ApiResponse, ApiTags } from '@nestjs/swagger';
import { FileInterceptor } from '@nestjs/platform-express';

@ApiTags('Image Processing')
@Controller('image-processing')
export class AppController {
  constructor(private readonly appService: AppService) {}

  @Post('rust')
  @UseInterceptors(FileInterceptor('file'))
  @ApiConsumes('multipart/form-data')
  @ApiBody({
    schema: {
      type: 'object',
      properties: {
        file: {
          type: 'string',
          format: 'binary',
        },
      },
    },
  })
  @ApiResponse({
    status: 200,
    description: 'The processed image.',
    content: { 'image/jpeg': {} },
  })
  async processImageRust(
    @UploadedFile() file: Express.Multer.File,
    @Res() res: Response,
  ): Promise<void> {
    const { result, durationMs } = await this.appService.processImageRust(
      file.buffer,
    );
    res.setHeader('Content-Type', 'image/jpeg');
    res.setHeader('X-Processing-Time-Ms', durationMs.toString());
    res.send(result);
  }

  @Post('javascript-sharp')
  @UseInterceptors(FileInterceptor('file'))
  @ApiConsumes('multipart/form-data')
  @ApiBody({
    schema: {
      type: 'object',
      properties: {
        file: {
          type: 'string',
          format: 'binary',
        },
      },
    },
  })
  @ApiResponse({
    status: 200,
    description: 'The processed image.',
    content: { 'image/jpeg': {} },
  })
  async processImageWithSharp(
    @UploadedFile() file: Express.Multer.File,
    @Res() res: Response,
  ): Promise<void> {
    const { result, durationMs } = await this.appService.processImageWithSharp(
      file.buffer,
    );
    res.setHeader('Content-Type', 'image/jpeg');
    res.setHeader('X-Processing-Time-Ms', durationMs.toString());
    res.send(result);
  }
}

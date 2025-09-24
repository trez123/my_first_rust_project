import { Injectable } from '@nestjs/common';
import Jimp from 'jimp';
import * as silasEcommerseFeatures from 'silas-ecommerse-features';
import sharp from 'sharp';
import * as process from 'process';

@Injectable()
export class AppService {
  async processImageRust(
    imageBytes: Uint8Array,
  ): Promise<{ result: Uint8Array; durationMs: number }> {
    const start = process.hrtime();

    const result = silasEcommerseFeatures.process_image(imageBytes);

    const diff = process.hrtime(start);
    const durationMs = diff[0] * 1000 + diff[1] / 1e6;

    return { result, durationMs };
  }

  async processImageWithSharp(
    imageBytes: Uint8Array,
  ): Promise<{ result: Uint8Array; durationMs: number }> {
    const start = process.hrtime();

    const buffer = await sharp(Buffer.from(imageBytes))
      .resize(800, 800, {
        fit: 'cover',
        position: 'left top', // Corresponds to crop(0, 0, ...)
      })
      .jpeg()
      .toBuffer();

    const diff = process.hrtime(start);
    const durationMs = diff[0] * 1000 + diff[1] / 1e6;

    return { result: new Uint8Array(buffer), durationMs };
  }
}

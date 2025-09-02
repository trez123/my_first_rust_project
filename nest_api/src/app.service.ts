import { Injectable } from '@nestjs/common';
import * as silasEcommerseFeatures from 'silas-ecommerse-features';

@Injectable()
export class AppService {
  async processImage(imageBytes: Uint8Array): Promise<Uint8Array> {
    const result = silasEcommerseFeatures.process_image(imageBytes);
    return result;
  }
  getHello(): string {
    return 'Hello World!';
  }
}

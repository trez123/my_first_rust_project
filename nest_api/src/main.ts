import { NestFactory } from '@nestjs/core';
import { AppModule } from './app.module';
import { DocumentBuilder, SwaggerModule } from '@nestjs/swagger';
const fs = require('fs');

async function bootstrap() {
  const app = await NestFactory.create(AppModule);
  app.enableCors({
    origin: [
      'http://localhost:9000',
      'http://localhost:8080',
      'http://localhost:60245',
    ],
    methods: ['GET', 'POST', 'PUT', 'DELETE', 'OPTIONS'],
    credentials: true,
  });

  // Only enable Swagger in non-production environments
  if (process.env.NODE_ENV !== 'production') {
    const config = new DocumentBuilder()
      .setTitle('Zendit gateway API Wrapper')
      .setVersion('1.0')
      .addBearerAuth(
        { type: 'http', scheme: 'bearer', bearerFormat: 'JWT' },
        'Authorization',
      )
      .addTag('Enpoints')
      .build();
    const document = SwaggerModule.createDocument(app, config);
    fs.writeFileSync('./swagger-spec.json', JSON.stringify(document));
    SwaggerModule.setup('/', app, document);
  }

  await app.listen(3001, '0.0.0.0');
}
bootstrap();

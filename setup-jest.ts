import 'jest-preset-angular/setup-jest';

HTMLCanvasElement.prototype.getContext = jest.fn().mockReturnValue(null);
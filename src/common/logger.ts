import toastr from 'toastr';
import 'toastr/build/toastr.min.css';

class Logger {
    private static instance: Logger;

    private constructor() {
        // Configure toastr options
        toastr.options = {
            positionClass: 'toast-bottom-right',
            timeOut: 5000,
            extendedTimeOut: 1000,
            closeButton: true,
            progressBar: true,
        };
    }

    public static getInstance(): Logger {
        if (!Logger.instance) {
            Logger.instance = new Logger();
        }
        return Logger.instance;
    }

    public info(message: string, ...optionalParams: any[]): void {
        console.info(`INFO: ${message}`, ...optionalParams);
    }

    public warn(message: string, ...optionalParams: any[]): void {
        console.warn(`WARN: ${message}`, ...optionalParams);
    }

    public error(message: string, ...optionalParams: any[]): void {
        console.error(`ERROR: ${message}`, ...optionalParams);
        toastr.error(message);
    }

    public trace(message: string, ...optionalParams: any[]): void {
        console.trace(`TRACE: ${message}`, ...optionalParams);
    }

    public debug(message: string, ...optionalParams: any[]): void {
        console.debug(`DEBUG: ${message}`, ...optionalParams);
    }
}

export default Logger;
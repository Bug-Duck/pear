declare namespace App {
  interface Remote {
    showDialog(message: string): Promise<void>
  }
}

interface Window {
  remote: App.Remote
}
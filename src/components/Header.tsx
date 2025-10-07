import { Search, Download, Upload } from 'lucide-react';
import { Input } from '@/components/ui/input';
import { Button } from '@/components/ui/button';
import { ThemeToggle } from './ThemeToggle';
import { useRef } from 'react';
import { HeaderProps } from '@/types';


export function Header({ searchQuery, onSearchChange, onExport, onImport }: HeaderProps) {
  const fileInputRef = useRef<HTMLInputElement>(null);

  const handleImportClick = () => {
    fileInputRef.current?.click();
  };

  const handleFileChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0];
    if (file) {
      onImport(file);
      e.target.value = '';
    }
  };

  return (
    <header className="border-b bg-card/50 backdrop-blur-sm sticky top-0 z-10">
      <div className="flex items-center gap-4 p-4">
        <div className="relative flex-1 max-w-md">
          <Search className="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-muted-foreground" />
          <Input
            placeholder="Search snippets..."
            value={searchQuery}
            onChange={(e) => onSearchChange(e.target.value)}
            className="pl-10"
          />
        </div>

        <div className="flex items-center gap-2">
          <Button variant="outline" size="icon" onClick={onExport} title="Export snippets">
            <Download className="w-4 h-4" />
          </Button>

          <Button variant="outline" size="icon" onClick={handleImportClick} title="Import snippets">
            <Upload className="w-4 h-4" />
          </Button>
          <input
            ref={fileInputRef}
            type="file"
            accept=".json"
            onChange={handleFileChange}
            className="hidden"
          />

          <ThemeToggle />
        </div>
      </div>
    </header>
  );
}
"use client";

import { useDebounce } from "@/hooks/use-debounce";
import { cn } from "@/lib/utils";
import { Search, X } from "lucide-react";
import { useEffect, useState } from "react";
import { Input } from "@/components/ui/input";

interface SearchInputProps {
  value?: string;
  onChange: (value: string) => void;
  placeholder?: string;
  delay?: number;
  className?: string;
}

export function SearchInput({
  value: initialValue = "",
  onChange,
  placeholder = "Cari data...",
  delay = 300,
  className,
}: SearchInputProps) {
  const [searchTerm, setSearchTerm] = useState(initialValue);
  const [prevInitial, setPrevInitial] = useState(initialValue);
  const debouncedSearch = useDebounce(searchTerm, delay);

  if (initialValue !== prevInitial) {
    setPrevInitial(initialValue);
    setSearchTerm(initialValue);
  }

  useEffect(() => {
    if (debouncedSearch !== initialValue) {
      onChange(debouncedSearch);
    }
  }, [debouncedSearch, onChange, initialValue]);

  return (
    <div
      className={cn("relative flex w-full max-w-xs items-center", className)}
    >
      <Search className="text-muted-foreground pointer-events-none absolute left-2.5 h-4 w-4" />
      <Input
        type="text"
        value={searchTerm}
        onChange={(e) => setSearchTerm(e.target.value)}
        placeholder={placeholder}
        className="border-border bg-card h-9 rounded-lg pr-8 pl-8 text-xs"
        aria-label={placeholder}
      />
      {searchTerm && (
        <button
          type="button"
          onClick={() => {
            setSearchTerm("");
            onChange("");
          }}
          className="text-muted-foreground hover:text-foreground absolute right-2.5"
          aria-label="Hapus teks pencarian"
        >
          <X className="h-3.5 w-3.5" />
        </button>
      )}
    </div>
  );
}

"use client";

import {
  Bold,
  Italic,
  Strikethrough,
  Code,
  Heading1,
  Heading2,
  Heading3,
  List,
  ListOrdered,
  Quote,
  Link as LinkIcon,
  type LucideIcon,
} from "lucide-react";
import { Button } from "@/components/ui/button";
import { Textarea } from "@/components/ui/textarea";
import { Tabs, TabsList, TabsTrigger, TabsContent } from "@/components/ui/tabs";
import { MarkdownRenderer } from "@/components/shared/markdown-renderer";
import { cn } from "@/lib/utils";
import {
  applyHeading,
  insertLink,
  toggleInlineWrap,
  toggleLinePrefix,
  toggleOrderedList,
  type EditorState,
} from "@/lib/markdown-toolbar";
import {
  forwardRef,
  Fragment,
  useCallback,
  useLayoutEffect,
  useRef,
  useState,
} from "react";

export interface MarkdownEditorProps {
  value: string;
  onChange: (value: string) => void;
  id?: string;
  name?: string;
  onBlur?: React.FocusEventHandler<HTMLTextAreaElement>;
  placeholder?: string;
  rows?: number;
  "aria-invalid"?: boolean;
  "aria-describedby"?: string;
}

interface ToolbarCommand {
  key: string;
  label: string;
  icon: LucideIcon;
  run: (state: EditorState) => EditorState;
  shortcut?: string;
}

const COMMAND_GROUPS: ToolbarCommand[][] = [
  [
    {
      key: "bold",
      label: "Tebal",
      icon: Bold,
      run: (s) => toggleInlineWrap(s, "**"),
      shortcut: "Ctrl+B",
    },
    {
      key: "italic",
      label: "Miring",
      icon: Italic,
      run: (s) => toggleInlineWrap(s, "*"),
      shortcut: "Ctrl+I",
    },
    {
      key: "strike",
      label: "Coret",
      icon: Strikethrough,
      run: (s) => toggleInlineWrap(s, "~~"),
    },
    {
      key: "code",
      label: "Kode",
      icon: Code,
      run: (s) => toggleInlineWrap(s, "`", "kode"),
    },
  ],
  [
    {
      key: "h1",
      label: "Tajuk 1",
      icon: Heading1,
      run: (s) => applyHeading(s, 1),
    },
    {
      key: "h2",
      label: "Tajuk 2",
      icon: Heading2,
      run: (s) => applyHeading(s, 2),
    },
    {
      key: "h3",
      label: "Tajuk 3",
      icon: Heading3,
      run: (s) => applyHeading(s, 3),
    },
  ],
  [
    {
      key: "ul",
      label: "Daftar poin",
      icon: List,
      run: (s) => toggleLinePrefix(s, "- "),
    },
    {
      key: "ol",
      label: "Daftar berurutan",
      icon: ListOrdered,
      run: (s) => toggleOrderedList(s),
    },
    {
      key: "quote",
      label: "Kutipan",
      icon: Quote,
      run: (s) => toggleLinePrefix(s, "> "),
    },
    { key: "link", label: "Tautan", icon: LinkIcon, run: (s) => insertLink(s) },
  ],
];

export const MarkdownEditor = forwardRef<
  HTMLTextAreaElement,
  MarkdownEditorProps
>(function MarkdownEditor(
  { value, onChange, id, name, onBlur, placeholder, rows = 14, ...aria },
  forwardedRef
) {
  const invalid = aria["aria-invalid"] ?? false;
  const innerRef = useRef<HTMLTextAreaElement | null>(null);
  const pendingSelection = useRef<[number, number] | null>(null);
  const [tab, setTab] = useState<"write" | "preview">("write");

  const setRef = useCallback(
    (node: HTMLTextAreaElement | null) => {
      innerRef.current = node;
      if (typeof forwardedRef === "function") forwardedRef(node);
      else if (forwardedRef) forwardedRef.current = node;
    },
    [forwardedRef]
  );

  // After a toolbar transform re-renders the controlled textarea, the caret
  // jumps to the end — restore the intended selection so editing feels native.
  useLayoutEffect(() => {
    const el = innerRef.current;
    if (!pendingSelection.current || !el) return;
    const [start, end] = pendingSelection.current;
    pendingSelection.current = null;
    el.focus();
    el.setSelectionRange(start, end);
  });

  const runCommand = useCallback(
    (command: ToolbarCommand) => {
      const el = innerRef.current;
      const next = command.run({
        value,
        selectionStart: el?.selectionStart ?? value.length,
        selectionEnd: el?.selectionEnd ?? value.length,
      });
      pendingSelection.current = [next.selectionStart, next.selectionEnd];
      onChange(next.value);
    },
    [value, onChange]
  );

  const handleKeyDown = useCallback(
    (event: React.KeyboardEvent<HTMLTextAreaElement>) => {
      if (!(event.metaKey || event.ctrlKey) || event.altKey || event.shiftKey)
        return;
      const key = event.key.toLowerCase();
      if (key === "b") {
        event.preventDefault();
        runCommand(COMMAND_GROUPS[0][0]);
      } else if (key === "i") {
        event.preventDefault();
        runCommand(COMMAND_GROUPS[0][1]);
      }
    },
    [runCommand]
  );

  const inPreview = tab === "preview";

  return (
    <div
      data-slot="markdown-editor"
      className={cn(
        "bg-card overflow-hidden rounded-lg border shadow-sm transition-colors",
        "focus-within:ring-ring focus-within:ring-2 focus-within:ring-offset-2",
        invalid
          ? "border-destructive focus-within:ring-destructive/30"
          : "border-input"
      )}
    >
      <Tabs
        value={tab}
        onValueChange={(next) => setTab(next as "write" | "preview")}
      >
        <div className="border-border bg-muted/40 flex flex-col gap-2 border-b px-2 py-1.5 sm:flex-row sm:items-center sm:justify-between">
          <div
            role="toolbar"
            aria-label="Format markdown"
            aria-controls={id}
            className={cn(
              "flex scrollbar-none items-center gap-0.5 overflow-x-auto py-1 [-ms-overflow-style:none] sm:py-0 [&::-webkit-scrollbar]:hidden",
              inPreview && "pointer-events-none opacity-40"
            )}
          >
            {COMMAND_GROUPS.map((group, groupIndex) => (
              <Fragment key={group[0].key}>
                {groupIndex > 0 && (
                  <span
                    aria-hidden="true"
                    className="bg-border mx-1 h-5 w-px shrink-0"
                  />
                )}
                {group.map((command) => {
                  const Icon = command.icon;
                  return (
                    <Button
                      key={command.key}
                      type="button"
                      variant="ghost"
                      size="icon-sm"
                      tabIndex={inPreview ? -1 : 0}
                      aria-label={command.label}
                      title={
                        command.shortcut
                          ? `${command.label} (${command.shortcut})`
                          : command.label
                      }
                      className="text-muted-foreground hover:text-foreground h-9 w-9 min-w-9 shrink-0 sm:h-8 sm:w-8"
                      onMouseDown={(event) => event.preventDefault()}
                      onClick={() => runCommand(command)}
                    >
                      <Icon className="h-4 w-4" aria-hidden="true" />
                    </Button>
                  );
                })}
              </Fragment>
            ))}
          </div>

          <TabsList className="h-11 shrink-0 self-end sm:h-8 sm:self-auto">
            <TabsTrigger value="write" className="h-full px-3 text-xs">
              Tulis
            </TabsTrigger>
            <TabsTrigger value="preview" className="h-full px-3 text-xs">
              Pratinjau
            </TabsTrigger>
          </TabsList>
        </div>

        <TabsContent value="write" className="mt-0">
          <Textarea
            ref={setRef}
            id={id}
            name={name}
            value={value}
            rows={rows}
            placeholder={placeholder}
            onChange={(event) => onChange(event.target.value)}
            onBlur={onBlur}
            onKeyDown={handleKeyDown}
            aria-invalid={invalid}
            aria-describedby={aria["aria-describedby"]}
            className="min-h-60 resize-y rounded-none border-0 bg-transparent p-3 font-mono text-sm shadow-none focus-visible:ring-0 focus-visible:ring-offset-0 sm:min-h-72 sm:p-4"
          />
        </TabsContent>

        <TabsContent
          value="preview"
          className="mt-0 min-h-60 overflow-auto p-3 focus-visible:ring-0 sm:min-h-72 sm:p-4"
        >
          {value.trim() ? (
            <MarkdownRenderer content={value} className="text-sm" />
          ) : (
            <p className="text-muted-foreground text-sm">
              Belum ada konten untuk dipratinjau.
            </p>
          )}
        </TabsContent>
      </Tabs>
    </div>
  );
});

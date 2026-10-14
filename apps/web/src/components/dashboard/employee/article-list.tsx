import Link from "next/link";
import { BookOpen } from "lucide-react";
import { EmptyState } from "@/components/shared/empty-state";
import { Skeleton } from "@/components/ui/skeleton";
import type { KnowledgeArticleListItem } from "@/types/articles";

interface EmployeeArticleListProps {
  articles: KnowledgeArticleListItem[];
  isLoading?: boolean;
}

export function EmployeeArticleList({
  articles,
  isLoading = false,
}: EmployeeArticleListProps) {
  if (isLoading) {
    return (
      <div className="space-y-3 py-1">
        <Skeleton className="h-12 w-full" />
        <Skeleton className="h-12 w-full" />
      </div>
    );
  }

  if (articles.length === 0) {
    return (
      <EmptyState
        title="Tidak Ada Artikel"
        description="Belum ada artikel terbaru."
      />
    );
  }

  return (
    <div className="divide-border divide-y">
      {articles.map((article) => (
        <Link
          key={article.id}
          href={`/knowledge/${article.slug}`}
          className="group hover:bg-muted/50 flex items-center justify-between rounded-sm px-1 py-3 transition-colors"
        >
          <div className="flex min-w-0 items-center gap-3">
            <div className="bg-muted/50 text-muted-foreground group-hover:text-foreground flex h-8 w-8 shrink-0 items-center justify-center rounded-lg transition-colors">
              <BookOpen className="h-4 w-4" strokeWidth={1.5} />
            </div>
            <div className="min-w-0">
              <div className="text-foreground truncate text-xs font-semibold group-hover:underline">
                {article.title}
              </div>
              <div className="text-muted-foreground text-[11px]">
                {article.category ? article.category.name : "Umum"}
              </div>
            </div>
          </div>
        </Link>
      ))}
    </div>
  );
}

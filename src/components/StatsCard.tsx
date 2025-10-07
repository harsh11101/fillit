import { StatsCardsProps } from '@/types';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { FileText, Tag, TrendingUp } from 'lucide-react';


export function StatsCards({ snippets }: StatsCardsProps) {
  const totalSnippets = snippets.length;
  const totalTags = new Set(snippets.flatMap((s) => s.tags)).size;
  const totalUsage = snippets.reduce((sum, s) => sum + s.usage_count, 0);

  const stats = [
    {
      title: 'Total Snippets',
      value: totalSnippets,
      icon: FileText,
      color: 'text-blue-500',
      bgColor: 'bg-blue-500/10',
    },
    {
      title: 'Tags',
      value: totalTags,
      icon: Tag,
      color: 'text-purple-500',
      bgColor: 'bg-purple-500/10',
    },
    {
      title: 'Total Usage',
      value: totalUsage,
      icon: TrendingUp,
      color: 'text-green-500',
      bgColor: 'bg-green-500/10',
    },
  ];

  return (
    <div className="grid gap-4 md:grid-cols-3 mb-6">
      {stats.map((stat) => (
        <Card key={stat.title}>
          <CardHeader className="flex flex-row items-center justify-between pb-2">
            <CardTitle className="text-sm font-medium text-muted-foreground">
              {stat.title}
            </CardTitle>
            <div className={`p-2 rounded-lg ${stat.bgColor}`}>
              <stat.icon className={`w-4 h-4 ${stat.color}`} />
            </div>
          </CardHeader>
          <CardContent>
            <div className="text-3xl font-bold">{stat.value}</div>
          </CardContent>
        </Card>
      ))}
    </div>
  );
}
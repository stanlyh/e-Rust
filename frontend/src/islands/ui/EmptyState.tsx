interface EmptyStateProps {
  icon?: string;
  title: string;
  description?: string;
  action?: { label: string; href?: string; onClick?: () => void };
}

export function EmptyState({ icon = '📭', title, description, action }: EmptyStateProps) {
  return (
    <div className="flex flex-col items-center justify-center py-16 text-center">
      <span className="text-4xl mb-3">{icon}</span>
      <p className="text-sm font-medium text-gray-700 mb-1">{title}</p>
      {description && <p className="text-xs text-gray-400 mb-4 max-w-xs">{description}</p>}
      {action && (
        action.href ? (
          <a href={action.href} className="text-sm text-blue-600 hover:underline font-medium">
            {action.label}
          </a>
        ) : (
          <button onClick={action.onClick} className="text-sm text-blue-600 hover:underline font-medium">
            {action.label}
          </button>
        )
      )}
    </div>
  );
}

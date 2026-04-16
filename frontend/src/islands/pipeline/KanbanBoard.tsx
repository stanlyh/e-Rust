import { useEffect, useState } from 'react';
import { toast } from '../../lib/toast';
import { ErrorState } from '../ui/ErrorState';
import {
  DndContext,
  DragOverlay,
  closestCorners,
  type DragEndEvent,
  type DragStartEvent,
  PointerSensor,
  useSensor,
  useSensors,
} from '@dnd-kit/core';
import { SortableContext, useSortable, verticalListSortingStrategy } from '@dnd-kit/sortable';
import { CSS } from '@dnd-kit/utilities';
import {
  opportunitiesApi,

  STATUS_COLORS,
  STATUS_LABELS,
  STAGE_ORDER,
  type Opportunity,
  type OpportunityStatus,
  type PipelineColumn,
} from '../../lib/api/opportunities';

// ── Tarjeta individual ────────────────────────────────────────────────────────
function KanbanCard({ opp, isDragging = false }: { opp: Opportunity; isDragging?: boolean }) {
  return (
    <div className={`bg-white rounded-lg border border-gray-200 p-3 shadow-sm select-none
      ${isDragging ? 'opacity-80 shadow-lg rotate-1' : 'hover:shadow-md transition-shadow'}`}
    >
      <p className="text-sm font-medium text-gray-800 truncate mb-1">{opp.title}</p>
      <div className="flex items-center justify-between">
        <span className="text-xs text-gray-500">{opp.probability}% prob.</span>
        {opp.offered_price && (
          <span className="text-xs font-semibold text-blue-600">
            ${Number(opp.offered_price).toLocaleString('es-MX')}
          </span>
        )}
      </div>
      {opp.expected_close && (
        <p className="text-xs text-gray-400 mt-1">
          Cierre: {new Date(opp.expected_close).toLocaleDateString('es-MX')}
        </p>
      )}
    </div>
  );
}

// ── Tarjeta sortable (con dnd-kit) ────────────────────────────────────────────
function SortableCard({ opp }: { opp: Opportunity }) {
  const { attributes, listeners, setNodeRef, transform, transition, isDragging } =
    useSortable({ id: opp.id, data: { opp } });

  const style = {
    transform: CSS.Transform.toString(transform),
    transition,
    opacity: isDragging ? 0.3 : 1,
  };

  return (
    <div ref={setNodeRef} style={style} {...attributes} {...listeners}>
      <KanbanCard opp={opp} />
    </div>
  );
}

// ── Columna del kanban ────────────────────────────────────────────────────────
function KanbanColumn({ column }: { column: PipelineColumn }) {
  const colorClass = STATUS_COLORS[column.status];
  return (
    <div className={`flex flex-col w-64 shrink-0 rounded-xl border-2 ${colorClass} overflow-hidden`}>
      {/* Header de columna */}
      <div className="p-3 border-b border-black/10">
        <div className="flex items-center justify-between">
          <h3 className="text-sm font-semibold text-gray-700">
            {STATUS_LABELS[column.status]}
          </h3>
          <span className="text-xs bg-white/60 rounded-full px-2 py-0.5 text-gray-600 font-medium">
            {column.count}
          </span>
        </div>
        {column.total_value > 0 && (
          <p className="text-xs text-gray-500 mt-0.5">
            ${column.total_value.toLocaleString('es-MX')} total
          </p>
        )}
      </div>

      {/* Cards */}
      <SortableContext items={column.opportunities.map(o => o.id)} strategy={verticalListSortingStrategy}>
        <div className="flex-1 p-2 space-y-2 min-h-20 overflow-y-auto max-h-[calc(100vh-280px)]">
          {column.opportunities.map(opp => (
            <SortableCard key={opp.id} opp={opp} />
          ))}
        </div>
      </SortableContext>
    </div>
  );
}

// ── Board principal ───────────────────────────────────────────────────────────
export default function KanbanBoard() {
  const [columns, setColumns] = useState<PipelineColumn[]>([]);
  const [activeOpp, setActiveOpp] = useState<Opportunity | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState('');

  const sensors = useSensors(
    useSensor(PointerSensor, { activationConstraint: { distance: 8 } })
  );

  const load = async () => {
    try {
      const res = await opportunitiesApi.pipeline();
      // Asegurar que todas las etapas existen aunque esten vacias
      const filled = STAGE_ORDER.map(status => {
        const col = res.columns.find(c => c.status === status);
        return col ?? { status, opportunities: [], total_value: 0, count: 0 };
      });
      setColumns(filled);
    } catch {
      setError('No se pudo cargar el pipeline.');
      toast.error('Error cargando el pipeline');
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => { load(); }, []);

  const handleDragStart = (event: DragStartEvent) => {
    const opp = event.active.data.current?.opp as Opportunity;
    setActiveOpp(opp ?? null);
  };

  const handleDragEnd = async (event: DragEndEvent) => {
    const { active, over } = event;
    setActiveOpp(null);
    if (!over) return;

    const draggedOpp = active.data.current?.opp as Opportunity;
    const targetStatus = over.id as OpportunityStatus;

    if (!draggedOpp || draggedOpp.status === targetStatus) return;

    // Optimistic update
    setColumns(prev => prev.map(col => {
      if (col.status === draggedOpp.status) {
        return {
          ...col,
          opportunities: col.opportunities.filter(o => o.id !== draggedOpp.id),
          count: col.count - 1,
          total_value: col.total_value - Number(draggedOpp.offered_price ?? 0),
        };
      }
      if (col.status === targetStatus) {
        const updated = { ...draggedOpp, status: targetStatus };
        return {
          ...col,
          opportunities: [...col.opportunities, updated],
          count: col.count + 1,
          total_value: col.total_value + Number(draggedOpp.offered_price ?? 0),
        };
      }
      return col;
    }));

    try {
      await opportunitiesApi.updateStatus(draggedOpp.id, targetStatus);
      toast.success(`Movido a ${targetStatus.replace('_', ' ')}`);
    } catch {
      toast.error('Error al mover la oportunidad');
      load();
    }
  };

  if (loading) return (
    <div className="flex gap-4">
      {STAGE_ORDER.map(s => (
        <div key={s} className="w-64 h-64 bg-gray-100 rounded-xl animate-pulse shrink-0" />
      ))}
    </div>
  );

  if (error) return <ErrorState message={error} onRetry={load} />;

  return (
    <DndContext
      sensors={sensors}
      collisionDetection={closestCorners}
      onDragStart={handleDragStart}
      onDragEnd={handleDragEnd}
    >
      <div className="flex gap-4 overflow-x-auto pb-4 h-full">
        {columns.map(col => (
          <KanbanColumn key={col.status} column={col} />
        ))}
      </div>

      <DragOverlay>
        {activeOpp && <KanbanCard opp={activeOpp} isDragging />}
      </DragOverlay>
    </DndContext>
  );
}

type WidgetSchema = {
  url: string;
  refresh_interval: number;
  level: "normal" | "alwaysontop" | "alwaysonbottom";
};

const defaultValues: WidgetSchema = {
  level: "normal",
  url: "",
  refresh_interval: 0,
};

export default function WidgetForm() {
  const handleSubmit = (event: React.FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    const formData = new FormData(event.currentTarget);
    const data = {
      url: formData.get("url") as string,
      level: formData.get("level") as WidgetSchema["level"],
      refresh_interval: formData.get("refresh_interval")
        ? Number(formData.get("refresh_interval"))
        : 0,
    };
    console.log("Form submitted:", data);
    window.ipc.postMessage(
      JSON.stringify({
        createwidget: {
          url: data.url,
          level: data.level,
          refresh_interval: data.refresh_interval,
        },
      })
    );
  };

  const handleReset = (event: React.FormEvent<HTMLFormElement>) => {
    event.currentTarget.reset();
  };

  const inputClass =
    "w-full px-2 h-8 bg-[#FDFD96] border-[3px] border-black text-lg focus:outline-none appearance-none";
  const labelClass =
    "block w-full bg-[#FF90BC] h-8 leading-7 border-x-[3px] border-t-[3px] border-black text-center font-black text-lg uppercase";

  return (
    <div className="p-2 max-w-md mx-auto">
      <div className="shadow-[6px_6px_0px_0px_rgba(0,0,0,1)]">
        <div className="bg-[#FF90BC] h-12 flex justify-center border-b-[3px] border-x-[3px] border-t-[3px] border-black">
          <h2 className="text-2xl font-black text-center uppercase tracking-[0.2em]">
            Create Widget
          </h2>
        </div>

        <form onSubmit={handleSubmit} onReset={handleReset}>
          {/* URL Field */}
          <div>
            <label htmlFor="url" className={labelClass}>
              URL
            </label>
            <input
              type="text"
              id="url"
              name="url"
              defaultValue={defaultValues.url}
              placeholder="https://example.com"
              className={`${inputClass} border-b-[3px]`}
              required
            />
          </div>

          {/* Window Level Field */}
          <div>
            <label htmlFor="level" className={labelClass}>
              Window Level
            </label>
            <select
              id="level"
              name="level"
              defaultValue={defaultValues.level}
              className={`${inputClass} border-b-[3px] cursor-pointer`}
            >
              <option value="normal">Normal</option>
              <option value="alwaysontop">Always on Top</option>
              <option value="alwaysonbottom">Always on Bottom</option>
            </select>
          </div>

          {/* Refresh Interval Field */}
          <div>
            <label htmlFor="refresh_interval" className={labelClass}>
              Refresh Interval (ms)
            </label>
            <input
              type="number"
              id="refresh_interval"
              name="refresh_interval"
              defaultValue={defaultValues.refresh_interval}
              min="0"
              placeholder="0"
              className={`${inputClass} border-b-[3px]`}
            />
          </div>

          {/* Form Actions */}
          <div className="flex justify-end bg-[#FF90BC] border-t-[3px] border-x-[3px] border-b-[3px] border-black">
            <button
              type="reset"
              className="h-10 px-6 text-lg font-black bg-[#98EECC] border-l-[3px] border-black uppercase hover:bg-[#7DCCAA] active:bg-[#98EECC] transition-colors"
            >
              Reset
            </button>
            <button
              type="submit"
              className="h-10 px-6 text-lg font-black bg-[#A7D2CB] border-l-[3px] border-black uppercase hover:bg-[#86B1AA] active:bg-[#A7D2CB] transition-colors"
            >
              Create Widget
            </button>
          </div>
        </form>
      </div>
    </div>
  );
}
